use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 250], OperandSize::Dword)
}

#[test]
fn pabsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(EDX, 315688932, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 154, 228, 7, 209, 18], OperandSize::Dword)
}

#[test]
fn pabsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 218], OperandSize::Qword)
}

#[test]
fn pabsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1620401104, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 20, 213, 208, 91, 149, 96], OperandSize::Qword)
}

#[test]
fn pabsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 204], OperandSize::Dword)
}

#[test]
fn pabsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 59], OperandSize::Dword)
}

#[test]
fn pabsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 223], OperandSize::Qword)
}

#[test]
fn pabsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 12, 78], OperandSize::Qword)
}

