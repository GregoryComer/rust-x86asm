use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 186, 245], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 1155041638, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 186, 155, 102, 137, 216, 68], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 186, 243], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 186, 42], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 186, 238], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDX, 1748243358, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 186, 186, 158, 19, 52, 104], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 186, 255], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1719322492, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 186, 52, 205, 124, 199, 122, 102], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 140, 186, 249], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 139, 186, 4, 78], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 153, 186, 2], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 157, 135, 186, 242], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RBX, 2001282989, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 133, 133, 186, 139, 173, 39, 73, 119], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1946393709, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 145, 186, 188, 142, 109, 156, 3, 116], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 171, 186, 198], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1797965377, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 186, 148, 90, 65, 198, 42, 107], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 2041337166, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 187, 186, 20, 69, 78, 85, 172, 121], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 165, 175, 186, 201], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 756866184, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 181, 161, 186, 60, 253, 136, 220, 28, 45], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 330928203, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 205, 179, 186, 4, 69, 75, 144, 185, 19], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 159, 186, 235], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ESI, 341954244, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 203, 186, 134, 196, 206, 97, 20], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 222, 186, 20, 73], OperandSize::Dword)
}

#[test]
fn vfmsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 165, 222, 186, 224], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 173, 202, 186, 20, 219], OperandSize::Qword)
}

#[test]
fn vfmsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectDisplaced(RCX, 1018979512, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 181, 220, 186, 137, 184, 100, 188, 60], OperandSize::Qword)
}

