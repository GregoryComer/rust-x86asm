use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsubadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 183, 240], OperandSize::Dword)
}

fn vfmsubadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 1254387847, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 183, 175, 135, 112, 196, 74], OperandSize::Dword)
}

fn vfmsubadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 183, 223], OperandSize::Qword)
}

fn vfmsubadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1810669516, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 183, 28, 117, 204, 159, 236, 107], OperandSize::Qword)
}

fn vfmsubadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 183, 232], OperandSize::Dword)
}

fn vfmsubadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 1780293403, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 183, 175, 27, 31, 29, 106], OperandSize::Dword)
}

fn vfmsubadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 183, 231], OperandSize::Qword)
}

fn vfmsubadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1732311846, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 183, 180, 183, 38, 251, 64, 103], OperandSize::Qword)
}

fn vfmsubadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 143, 183, 244], OperandSize::Dword)
}

fn vfmsubadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 138, 183, 34], OperandSize::Dword)
}

fn vfmsubadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 553577064, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 158, 183, 36, 245, 104, 234, 254, 32], OperandSize::Dword)
}

fn vfmsubadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 101, 137, 183, 195], OperandSize::Qword)
}

fn vfmsubadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 133, 183, 36, 121], OperandSize::Qword)
}

fn vfmsubadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM17)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 117, 145, 183, 38], OperandSize::Qword)
}

fn vfmsubadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 175, 183, 210], OperandSize::Dword)
}

fn vfmsubadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDX, 131047061, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 173, 183, 154, 149, 158, 207, 7], OperandSize::Dword)
}

fn vfmsubadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1896495161, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 185, 183, 20, 149, 57, 56, 10, 113], OperandSize::Dword)
}

fn vfmsubadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 125, 169, 183, 227], OperandSize::Qword)
}

fn vfmsubadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 101, 175, 183, 48], OperandSize::Qword)
}

fn vfmsubadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectDisplaced(RAX, 592028260, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 29, 181, 183, 168, 100, 162, 73, 35], OperandSize::Qword)
}

fn vfmsubadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 157, 183, 245], OperandSize::Dword)
}

fn vfmsubadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EAX, 1805978656, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 206, 183, 136, 32, 12, 165, 107], OperandSize::Dword)
}

fn vfmsubadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 222, 183, 4, 146], OperandSize::Dword)
}

fn vfmsubadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 21, 181, 183, 240], OperandSize::Qword)
}

fn vfmsubadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM12)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 29, 206, 183, 32], OperandSize::Qword)
}

fn vfmsubadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 501536926, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 53, 215, 183, 148, 145, 158, 216, 228, 29], OperandSize::Qword)
}

