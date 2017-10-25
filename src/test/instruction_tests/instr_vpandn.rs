use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 223, 200], OperandSize::Dword)
}

#[test]
fn vpandn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 524395593, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 223, 148, 150, 73, 164, 65, 31], OperandSize::Dword)
}

#[test]
fn vpandn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 223, 215], OperandSize::Qword)
}

#[test]
fn vpandn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RBX, 882050872, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 223, 171, 56, 7, 147, 52], OperandSize::Qword)
}

#[test]
fn vpandn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 223, 253], OperandSize::Dword)
}

#[test]
fn vpandn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1288497734, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 223, 140, 86, 70, 234, 204, 76], OperandSize::Dword)
}

#[test]
fn vpandn_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 223, 201], OperandSize::Qword)
}

#[test]
fn vpandn_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 223, 44, 143], OperandSize::Qword)
}

