use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmullq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 140, 64, 222], OperandSize::Dword)
}

fn vpmullq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1758422147, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 141, 64, 188, 150, 131, 100, 207, 104], OperandSize::Dword)
}

fn vpmullq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 877498906, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 157, 64, 4, 85, 26, 146, 77, 52], OperandSize::Dword)
}

fn vpmullq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 221, 130, 64, 199], OperandSize::Qword)
}

fn vpmullq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1959530681, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 64, 44, 117, 185, 16, 204, 116], OperandSize::Qword)
}

fn vpmullq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RAX, 804121059, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 181, 146, 64, 160, 227, 233, 237, 47], OperandSize::Qword)
}

fn vpmullq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 64, 210], OperandSize::Dword)
}

fn vpmullq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 64, 52, 66], OperandSize::Dword)
}

fn vpmullq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EAX, 276073328, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 190, 64, 168, 112, 139, 116, 16], OperandSize::Dword)
}

fn vpmullq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 237, 172, 64, 216], OperandSize::Qword)
}

fn vpmullq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 500874551, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 205, 174, 64, 12, 85, 55, 189, 218, 29], OperandSize::Qword)
}

fn vpmullq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectDisplaced(RBX, 612952646, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 181, 64, 163, 70, 234, 136, 36], OperandSize::Qword)
}

fn vpmullq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 202, 64, 237], OperandSize::Dword)
}

fn vpmullq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EDX, 1125759716, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 202, 64, 154, 228, 186, 25, 67], OperandSize::Dword)
}

fn vpmullq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EAX, 107712143, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 220, 64, 176, 143, 142, 107, 6], OperandSize::Dword)
}

fn vpmullq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 197, 193, 64, 195], OperandSize::Qword)
}

fn vpmullq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 181, 206, 64, 12, 75], OperandSize::Qword)
}

fn vpmullq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 157, 209, 64, 25], OperandSize::Qword)
}

