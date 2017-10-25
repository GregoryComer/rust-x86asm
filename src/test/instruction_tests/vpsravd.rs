use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsravd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 70, 236], OperandSize::Dword)
}

fn vpsravd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1871945961, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 70, 188, 242, 233, 160, 147, 111], OperandSize::Dword)
}

fn vpsravd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 70, 234], OperandSize::Qword)
}

fn vpsravd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 70, 0], OperandSize::Qword)
}

fn vpsravd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 70, 193], OperandSize::Dword)
}

fn vpsravd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 70, 60, 154], OperandSize::Dword)
}

fn vpsravd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 70, 236], OperandSize::Qword)
}

fn vpsravd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 70, 36, 209], OperandSize::Qword)
}

fn vpsravd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 139, 70, 254], OperandSize::Dword)
}

fn vpsravd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 342759063, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 143, 70, 52, 253, 151, 22, 110, 20], OperandSize::Dword)
}

fn vpsravd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1162076108, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 155, 70, 155, 204, 223, 67, 69], OperandSize::Dword)
}

fn vpsravd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 109, 129, 70, 209], OperandSize::Qword)
}

fn vpsravd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RCX, 491992547, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 5, 133, 70, 161, 227, 53, 83, 29], OperandSize::Qword)
}

fn vpsravd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RDI, 585081945, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 45, 148, 70, 159, 89, 164, 223, 34], OperandSize::Qword)
}

fn vpsravd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 70, 236], OperandSize::Dword)
}

fn vpsravd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 311631108, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 70, 12, 189, 4, 29, 147, 18], OperandSize::Dword)
}

fn vpsravd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 191, 70, 24], OperandSize::Dword)
}

fn vpsravd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 109, 173, 70, 220], OperandSize::Qword)
}

fn vpsravd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectDisplaced(RAX, 463969381, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 77, 165, 70, 144, 101, 156, 167, 27], OperandSize::Qword)
}

fn vpsravd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 29, 188, 70, 36, 74], OperandSize::Qword)
}

fn vpsravd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 70, 194], OperandSize::Dword)
}

fn vpsravd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1698016440, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 205, 70, 44, 221, 184, 172, 53, 101], OperandSize::Dword)
}

fn vpsravd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 222, 70, 28, 153], OperandSize::Dword)
}

fn vpsravd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 53, 197, 70, 208], OperandSize::Qword)
}

fn vpsravd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 201, 70, 12, 113], OperandSize::Qword)
}

fn vpsravd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 53, 209, 70, 19], OperandSize::Qword)
}

