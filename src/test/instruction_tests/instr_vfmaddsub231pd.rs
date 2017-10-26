use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 182, 232], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 528191646, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 182, 60, 189, 158, 144, 123, 31], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 182, 216], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 182, 23], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 182, 217], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 182, 38], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 182, 233], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 182, 44, 78], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 137, 182, 225], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 182, 2], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 153, 182, 33], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 181, 133, 182, 240], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RCX, 1318248453, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 189, 143, 182, 169, 5, 224, 146, 78], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 148, 182, 62], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 169, 182, 250], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 68940160, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 174, 182, 4, 117, 128, 241, 27, 4], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 633495665, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 189, 182, 28, 245, 113, 96, 194, 37], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 165, 164, 182, 192], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectDisplaced(RBX, 105177555, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 167, 182, 187, 211, 225, 68, 6], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 205302577, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 149, 181, 182, 148, 66, 49, 171, 60, 12], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 218, 182, 253], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 204, 182, 60, 182], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 712754867, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 218, 182, 172, 145, 179, 198, 123, 42], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 149, 155, 182, 217], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 197, 199, 182, 60, 71], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 2028023299, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 237, 220, 182, 28, 213, 3, 46, 225, 120], OperandSize::Qword)
}

