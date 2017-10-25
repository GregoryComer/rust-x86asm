use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pandn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 198], OperandSize::Dword)
}

#[test]
fn pandn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 28, 187], OperandSize::Dword)
}

#[test]
fn pandn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 222], OperandSize::Qword)
}

#[test]
fn pandn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 54], OperandSize::Qword)
}

#[test]
fn pandn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 224], OperandSize::Dword)
}

#[test]
fn pandn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1519050434, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 28, 221, 194, 222, 138, 90], OperandSize::Dword)
}

#[test]
fn pandn_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 213], OperandSize::Qword)
}

#[test]
fn pandn_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RSI, 542820844, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 166, 236, 201, 90, 32], OperandSize::Qword)
}

