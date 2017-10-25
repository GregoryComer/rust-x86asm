use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsllvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 71, 206], OperandSize::Dword)
}

fn vpsllvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1330697804, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 71, 148, 242, 76, 214, 80, 79], OperandSize::Dword)
}

fn vpsllvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 71, 250], OperandSize::Qword)
}

fn vpsllvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1312956037, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 71, 172, 115, 133, 30, 66, 78], OperandSize::Qword)
}

fn vpsllvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 71, 228], OperandSize::Dword)
}

fn vpsllvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 1990183447, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 71, 162, 23, 202, 159, 118], OperandSize::Dword)
}

fn vpsllvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 71, 203], OperandSize::Qword)
}

fn vpsllvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 71, 10], OperandSize::Qword)
}

fn vpsllvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 143, 71, 236], OperandSize::Dword)
}

fn vpsllvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1038553039, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 143, 71, 52, 189, 207, 15, 231, 61], OperandSize::Dword)
}

fn vpsllvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 159, 71, 52, 118], OperandSize::Dword)
}

fn vpsllvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 133, 135, 71, 223], OperandSize::Qword)
}

fn vpsllvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 636697782, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 157, 138, 71, 156, 131, 182, 60, 243, 37], OperandSize::Qword)
}

fn vpsllvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 253, 148, 71, 20, 72], OperandSize::Qword)
}

fn vpsllvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 174, 71, 221], OperandSize::Dword)
}

fn vpsllvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 1255364453, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 174, 71, 172, 136, 101, 87, 211, 74], OperandSize::Dword)
}

fn vpsllvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 186, 71, 26], OperandSize::Dword)
}

fn vpsllvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 245, 161, 71, 250], OperandSize::Qword)
}

fn vpsllvq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 253, 175, 71, 36, 222], OperandSize::Qword)
}

fn vpsllvq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 180, 71, 52, 183], OperandSize::Qword)
}

fn vpsllvq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 201, 71, 252], OperandSize::Dword)
}

fn vpsllvq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 203, 71, 60, 138], OperandSize::Dword)
}

fn vpsllvq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDI, 1096745241, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 219, 71, 151, 25, 1, 95, 65], OperandSize::Dword)
}

fn vpsllvq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 237, 199, 71, 235], OperandSize::Qword)
}

fn vpsllvq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(RCX, 896557839, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 245, 202, 71, 137, 15, 99, 112, 53], OperandSize::Qword)
}

fn vpsllvq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1666695052, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 215, 71, 172, 201, 140, 191, 87, 99], OperandSize::Qword)
}

