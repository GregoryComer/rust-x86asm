use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vhaddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 124, 202], OperandSize::Dword)
}

#[test]
fn vhaddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 124, 0], OperandSize::Dword)
}

#[test]
fn vhaddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 124, 236], OperandSize::Qword)
}

#[test]
fn vhaddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDX, 1673144444, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 124, 178, 124, 40, 186, 99], OperandSize::Qword)
}

#[test]
fn vhaddps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 247, 124, 193], OperandSize::Dword)
}

#[test]
fn vhaddps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 124, 22], OperandSize::Dword)
}

#[test]
fn vhaddps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 207, 124, 222], OperandSize::Qword)
}

#[test]
fn vhaddps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHADDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 215, 124, 27], OperandSize::Qword)
}

