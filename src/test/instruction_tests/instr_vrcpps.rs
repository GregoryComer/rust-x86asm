use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 218], OperandSize::Dword)
}

#[test]
fn vrcpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDX, 778448398, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 178, 14, 46, 102, 46], OperandSize::Dword)
}

#[test]
fn vrcpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 247], OperandSize::Qword)
}

#[test]
fn vrcpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1986804542, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 83, 188, 152, 62, 59, 108, 118], OperandSize::Qword)
}

#[test]
fn vrcpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 233], OperandSize::Dword)
}

#[test]
fn vrcpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 206193504, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 36, 221, 96, 67, 74, 12], OperandSize::Dword)
}

#[test]
fn vrcpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 207], OperandSize::Qword)
}

#[test]
fn vrcpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPPS, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 83, 31], OperandSize::Qword)
}

