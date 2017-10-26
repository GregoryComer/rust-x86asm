use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 223, 199], OperandSize::Dword)
}

#[test]
fn vpandn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 223, 10], OperandSize::Dword)
}

#[test]
fn vpandn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 223, 230], OperandSize::Qword)
}

#[test]
fn vpandn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 223, 54], OperandSize::Qword)
}

#[test]
fn vpandn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 223, 198], OperandSize::Dword)
}

#[test]
fn vpandn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1246202685, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 223, 28, 181, 61, 139, 71, 74], OperandSize::Dword)
}

#[test]
fn vpandn_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 223, 229], OperandSize::Qword)
}

#[test]
fn vpandn_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RCX, 113802300, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 223, 161, 60, 124, 200, 6], OperandSize::Qword)
}

