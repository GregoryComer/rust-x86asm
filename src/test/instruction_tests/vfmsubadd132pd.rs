use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsubadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 151, 225], OperandSize::Dword)
}

fn vfmsubadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 151, 48], OperandSize::Dword)
}

fn vfmsubadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 151, 204], OperandSize::Qword)
}

fn vfmsubadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 238689206, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 151, 172, 150, 182, 27, 58, 14], OperandSize::Qword)
}

fn vfmsubadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 151, 254], OperandSize::Dword)
}

fn vfmsubadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ECX, 3041516, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 151, 161, 236, 104, 46, 0], OperandSize::Dword)
}

fn vfmsubadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 151, 206], OperandSize::Qword)
}

fn vfmsubadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 151, 27], OperandSize::Qword)
}

fn vfmsubadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 143, 151, 208], OperandSize::Dword)
}

fn vfmsubadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 608107301, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 151, 137, 37, 251, 62, 36], OperandSize::Dword)
}

fn vfmsubadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1038986039, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 156, 151, 60, 117, 55, 171, 237, 61], OperandSize::Dword)
}

fn vfmsubadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 245, 142, 151, 239], OperandSize::Qword)
}

fn vfmsubadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectDisplaced(RCX, 1354415244, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 149, 133, 151, 185, 140, 188, 186, 80], OperandSize::Qword)
}

fn vfmsubadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RSI, 132173097, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 157, 150, 151, 150, 41, 205, 224, 7], OperandSize::Qword)
}

fn vfmsubadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 151, 211], OperandSize::Dword)
}

fn vfmsubadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 151, 52, 80], OperandSize::Dword)
}

fn vfmsubadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 191, 151, 44, 158], OperandSize::Dword)
}

fn vfmsubadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 237, 173, 151, 250], OperandSize::Qword)
}

fn vfmsubadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RDX, 1725754538, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 141, 164, 151, 178, 170, 236, 220, 102], OperandSize::Qword)
}

fn vfmsubadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM15)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 133, 186, 151, 50], OperandSize::Qword)
}

fn vfmsubadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 187, 151, 215], OperandSize::Dword)
}

fn vfmsubadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 204, 151, 4, 247], OperandSize::Dword)
}

fn vfmsubadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDX, 1852572482, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 221, 151, 186, 66, 3, 108, 110], OperandSize::Dword)
}

fn vfmsubadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 237, 211, 151, 192], OperandSize::Qword)
}

fn vfmsubadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectDisplaced(RAX, 1845490707, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 181, 196, 151, 184, 19, 244, 255, 109], OperandSize::Qword)
}

fn vfmsubadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 215, 151, 4, 214], OperandSize::Qword)
}

