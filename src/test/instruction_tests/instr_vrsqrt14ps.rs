use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 78, 221], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 2116877857, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 78, 20, 197, 33, 254, 44, 126], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 159, 78, 30], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 125, 142, 78, 219], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 143, 78, 60, 120], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1716253436, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 155, 78, 44, 149, 252, 242, 75, 102], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 78, 231], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 450768615, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 78, 148, 202, 231, 46, 222, 26], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EBX, 1562473434, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 78, 147, 218, 115, 33, 93], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 175, 78, 212], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 693390581, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 175, 78, 180, 219, 245, 76, 84, 41], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1996924806, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 188, 78, 180, 190, 134, 167, 6, 119], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 78, 228], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 78, 12, 176], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 219, 78, 28, 248], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 202, 78, 244], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM19)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 78, 26], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1911248651, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 220, 78, 172, 186, 11, 87, 235, 113], OperandSize::Qword)
}

