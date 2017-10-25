use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaxuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 63, 238], OperandSize::Dword)
}

fn vpmaxuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 464390170, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 140, 63, 12, 141, 26, 8, 174, 27], OperandSize::Dword)
}

fn vpmaxuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1159821743, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 159, 63, 28, 253, 175, 121, 33, 69], OperandSize::Dword)
}

fn vpmaxuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 205, 133, 63, 232], OperandSize::Qword)
}

fn vpmaxuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 157, 141, 63, 36, 127], OperandSize::Qword)
}

fn vpmaxuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1704622068, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 221, 148, 63, 148, 216, 244, 119, 154, 101], OperandSize::Qword)
}

fn vpmaxuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 171, 63, 193], OperandSize::Dword)
}

fn vpmaxuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1139065056, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 171, 63, 36, 213, 224, 192, 228, 67], OperandSize::Dword)
}

fn vpmaxuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 817183739, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 186, 63, 12, 181, 251, 59, 181, 48], OperandSize::Dword)
}

fn vpmaxuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 165, 165, 63, 223], OperandSize::Qword)
}

fn vpmaxuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 229, 164, 63, 19], OperandSize::Qword)
}

fn vpmaxuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 292588395, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 157, 187, 63, 12, 141, 107, 139, 112, 17], OperandSize::Qword)
}

fn vpmaxuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 207, 63, 206], OperandSize::Dword)
}

fn vpmaxuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 205, 63, 12, 131], OperandSize::Dword)
}

fn vpmaxuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EDX, 1173181943, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 217, 63, 162, 247, 85, 237, 69], OperandSize::Dword)
}

fn vpmaxuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 197, 206, 63, 214], OperandSize::Qword)
}

fn vpmaxuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 173, 201, 63, 28, 153], OperandSize::Qword)
}

fn vpmaxuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 221, 214, 63, 56], OperandSize::Qword)
}

