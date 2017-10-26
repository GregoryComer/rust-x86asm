use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 238], OperandSize::Dword)
}

#[test]
fn vrcpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 2], OperandSize::Dword)
}

#[test]
fn vrcpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 196], OperandSize::Qword)
}

#[test]
fn vrcpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 28, 123], OperandSize::Qword)
}

#[test]
fn vrcpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 231], OperandSize::Dword)
}

#[test]
fn vrcpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1991608561, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 164, 65, 241, 136, 181, 118], OperandSize::Dword)
}

#[test]
fn vrcpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 218], OperandSize::Qword)
}

#[test]
fn vrcpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(RAX, 296850961, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 184, 17, 150, 177, 17], OperandSize::Qword)
}

