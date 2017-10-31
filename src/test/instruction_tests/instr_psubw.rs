use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 221], OperandSize::Dword)
}

#[test]
fn psubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1981587243, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 60, 125, 43, 159, 28, 118], OperandSize::Dword)
}

#[test]
fn psubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 245], OperandSize::Qword)
}

#[test]
fn psubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 11], OperandSize::Qword)
}

#[test]
fn psubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 233], OperandSize::Dword)
}

#[test]
fn psubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1889588455, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 4, 77, 231, 212, 160, 112], OperandSize::Dword)
}

#[test]
fn psubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 195], OperandSize::Qword)
}

#[test]
fn psubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RSI, 323693285, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 174, 229, 42, 75, 19], OperandSize::Qword)
}

