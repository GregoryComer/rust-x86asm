use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 223, 252], OperandSize::Dword)
}

#[test]
fn vpandn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 530859791, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 223, 12, 205, 15, 71, 164, 31], OperandSize::Dword)
}

#[test]
fn vpandn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 223, 248], OperandSize::Qword)
}

#[test]
fn vpandn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RAX, 732711283, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 223, 176, 115, 73, 172, 43], OperandSize::Qword)
}

#[test]
fn vpandn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 223, 195], OperandSize::Dword)
}

#[test]
fn vpandn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 223, 39], OperandSize::Dword)
}

#[test]
fn vpandn_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 223, 206], OperandSize::Qword)
}

#[test]
fn vpandn_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 80285458, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 223, 12, 149, 18, 15, 201, 4], OperandSize::Qword)
}

