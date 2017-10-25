use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmaddsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 150, 198], OperandSize::Dword)
}

fn vfmaddsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 150, 36, 223], OperandSize::Dword)
}

fn vfmaddsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 150, 255], OperandSize::Qword)
}

fn vfmaddsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RCX, 1620499350, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 150, 169, 150, 219, 150, 96], OperandSize::Qword)
}

fn vfmaddsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 150, 194], OperandSize::Dword)
}

fn vfmaddsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 150, 36, 153], OperandSize::Dword)
}

fn vfmaddsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 150, 226], OperandSize::Qword)
}

fn vfmaddsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RSI, 1715353088, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 150, 174, 0, 54, 62, 102], OperandSize::Qword)
}

fn vfmaddsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 140, 150, 216], OperandSize::Dword)
}

fn vfmaddsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1189937299, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 150, 140, 72, 147, 0, 237, 70], OperandSize::Dword)
}

fn vfmaddsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 251980537, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 155, 150, 52, 149, 249, 234, 4, 15], OperandSize::Dword)
}

fn vfmaddsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 117, 139, 150, 226], OperandSize::Qword)
}

fn vfmaddsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 415510721, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 133, 150, 164, 209, 193, 48, 196, 24], OperandSize::Qword)
}

fn vfmaddsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1914690889, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 117, 155, 150, 60, 205, 73, 221, 31, 114], OperandSize::Qword)
}

fn vfmaddsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 174, 150, 220], OperandSize::Dword)
}

fn vfmaddsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 150, 32], OperandSize::Dword)
}

fn vfmaddsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1182519572, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 190, 150, 52, 125, 20, 209, 123, 70], OperandSize::Dword)
}

fn vfmaddsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 5, 162, 150, 225], OperandSize::Qword)
}

fn vfmaddsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectDisplaced(RCX, 1707923157, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 5, 164, 150, 169, 213, 214, 204, 101], OperandSize::Qword)
}

fn vfmaddsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 190, 150, 59], OperandSize::Qword)
}

fn vfmaddsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 190, 150, 232], OperandSize::Dword)
}

fn vfmaddsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 203, 150, 12, 178], OperandSize::Dword)
}

fn vfmaddsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EBX, 739934517, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 218, 150, 147, 53, 129, 26, 44], OperandSize::Dword)
}

fn vfmaddsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 45, 250, 150, 251], OperandSize::Qword)
}

fn vfmaddsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 53, 197, 150, 52, 79], OperandSize::Qword)
}

fn vfmaddsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 5, 214, 150, 41], OperandSize::Qword)
}

