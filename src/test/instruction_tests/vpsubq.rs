use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsubq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 251, 248], OperandSize::Dword)
}

fn vpsubq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 495537090, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 251, 12, 133, 194, 75, 137, 29], OperandSize::Dword)
}

fn vpsubq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 251, 236], OperandSize::Qword)
}

fn vpsubq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RAX, 1299071907, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 251, 144, 163, 67, 110, 77], OperandSize::Qword)
}

fn vpsubq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 251, 231], OperandSize::Dword)
}

fn vpsubq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 1933864050, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 251, 176, 114, 108, 68, 115], OperandSize::Dword)
}

fn vpsubq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 251, 198], OperandSize::Qword)
}

fn vpsubq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 251, 60, 223], OperandSize::Qword)
}

fn vpsubq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 141, 251, 194], OperandSize::Dword)
}

fn vpsubq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 450625570, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 142, 251, 186, 34, 0, 220, 26], OperandSize::Dword)
}

fn vpsubq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 158, 251, 52, 64], OperandSize::Dword)
}

fn vpsubq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 221, 143, 251, 233], OperandSize::Qword)
}

fn vpsubq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 722458254, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 173, 132, 251, 28, 157, 142, 214, 15, 43], OperandSize::Qword)
}

fn vpsubq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RCX, 785301615, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 133, 148, 251, 161, 111, 192, 206, 46], OperandSize::Qword)
}

fn vpsubq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 169, 251, 202], OperandSize::Dword)
}

fn vpsubq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 171, 251, 36, 150], OperandSize::Dword)
}

fn vpsubq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 1688514349, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 191, 251, 148, 70, 45, 175, 164, 100], OperandSize::Dword)
}

fn vpsubq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 157, 164, 251, 225], OperandSize::Qword)
}

fn vpsubq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RSI, 860128032, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 237, 166, 251, 174, 32, 131, 68, 51], OperandSize::Qword)
}

fn vpsubq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 141, 182, 251, 18], OperandSize::Qword)
}

fn vpsubq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 229, 207, 251, 240], OperandSize::Dword)
}

fn vpsubq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 204, 251, 52, 118], OperandSize::Dword)
}

fn vpsubq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 219, 251, 52, 67], OperandSize::Dword)
}

fn vpsubq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 133, 198, 251, 253], OperandSize::Qword)
}

fn vpsubq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 157, 202, 251, 20, 182], OperandSize::Qword)
}

fn vpsubq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1648845304, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 133, 213, 251, 140, 114, 248, 97, 71, 98], OperandSize::Qword)
}

