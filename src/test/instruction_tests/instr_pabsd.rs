use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 200], OperandSize::Dword)
}

#[test]
fn pabsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 2071249704, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 60, 69, 40, 195, 116, 123], OperandSize::Dword)
}

#[test]
fn pabsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 212], OperandSize::Qword)
}

#[test]
fn pabsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 395651645, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 148, 127, 61, 42, 149, 23], OperandSize::Qword)
}

#[test]
fn pabsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 236], OperandSize::Dword)
}

#[test]
fn pabsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 25], OperandSize::Dword)
}

#[test]
fn pabsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 216], OperandSize::Qword)
}

#[test]
fn pabsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 34], OperandSize::Qword)
}

