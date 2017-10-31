use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackssdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 107, 212], OperandSize::Dword)
}

#[test]
fn vpackssdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 534999888, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 107, 164, 119, 80, 115, 227, 31], OperandSize::Dword)
}

#[test]
fn vpackssdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 107, 244], OperandSize::Qword)
}

#[test]
fn vpackssdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDX, 1337809486, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 107, 130, 78, 90, 189, 79], OperandSize::Qword)
}

#[test]
fn vpackssdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 107, 254], OperandSize::Dword)
}

#[test]
fn vpackssdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 425029862, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 107, 28, 69, 230, 112, 85, 25], OperandSize::Dword)
}

#[test]
fn vpackssdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 107, 217], OperandSize::Qword)
}

#[test]
fn vpackssdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 107, 60, 94], OperandSize::Qword)
}

#[test]
fn vpackssdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 107, 246], OperandSize::Dword)
}

#[test]
fn vpackssdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 140, 107, 52, 147], OperandSize::Dword)
}

#[test]
fn vpackssdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1335701403, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 155, 107, 20, 149, 155, 47, 157, 79], OperandSize::Dword)
}

#[test]
fn vpackssdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 5, 132, 107, 243], OperandSize::Qword)
}

#[test]
fn vpackssdw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 34140772, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 117, 143, 107, 135, 100, 242, 8, 2], OperandSize::Qword)
}

#[test]
fn vpackssdw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectDisplaced(RSI, 896050136, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 21, 147, 107, 190, 216, 163, 104, 53], OperandSize::Qword)
}

