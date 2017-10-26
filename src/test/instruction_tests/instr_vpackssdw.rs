use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackssdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 107, 226], OperandSize::Dword)
}

#[test]
fn vpackssdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1475092580, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 107, 60, 133, 100, 32, 236, 87], OperandSize::Dword)
}

#[test]
fn vpackssdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 107, 238], OperandSize::Qword)
}

#[test]
fn vpackssdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 718940561, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 107, 12, 133, 145, 41, 218, 42], OperandSize::Qword)
}

#[test]
fn vpackssdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 107, 208], OperandSize::Dword)
}

#[test]
fn vpackssdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ESI, 1558833111, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 107, 134, 215, 231, 233, 92], OperandSize::Dword)
}

#[test]
fn vpackssdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 107, 209], OperandSize::Qword)
}

#[test]
fn vpackssdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 107, 6], OperandSize::Qword)
}

#[test]
fn vpackssdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 107, 239], OperandSize::Dword)
}

#[test]
fn vpackssdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 107, 8], OperandSize::Dword)
}

#[test]
fn vpackssdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 154, 107, 32], OperandSize::Dword)
}

#[test]
fn vpackssdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 45, 132, 107, 216], OperandSize::Qword)
}

#[test]
fn vpackssdw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 101, 131, 107, 43], OperandSize::Qword)
}

#[test]
fn vpackssdw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 155, 107, 20, 154], OperandSize::Qword)
}

