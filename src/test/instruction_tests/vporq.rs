use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vporq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 142, 235, 223], OperandSize::Dword)
}

fn vporq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 142, 235, 43], OperandSize::Dword)
}

fn vporq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1040610874, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 157, 235, 36, 149, 58, 118, 6, 62], OperandSize::Dword)
}

fn vporq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 189, 137, 235, 232], OperandSize::Qword)
}

fn vporq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDX, 940332590, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 205, 142, 235, 154, 46, 86, 12, 56], OperandSize::Qword)
}

fn vporq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 149, 148, 235, 57], OperandSize::Qword)
}

fn vporq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 173, 235, 210], OperandSize::Dword)
}

fn vporq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 235, 36, 151], OperandSize::Dword)
}

fn vporq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 185, 235, 41], OperandSize::Dword)
}

fn vporq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 149, 163, 235, 207], OperandSize::Qword)
}

fn vporq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 245, 173, 235, 36, 74], OperandSize::Qword)
}

fn vporq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 237, 178, 235, 60, 184], OperandSize::Qword)
}

fn vporq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 205, 235, 220], OperandSize::Dword)
}

fn vporq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1451514239, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 205, 235, 188, 190, 127, 89, 132, 86], OperandSize::Dword)
}

fn vporq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 657842546, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 218, 235, 52, 77, 114, 225, 53, 39], OperandSize::Dword)
}

fn vporq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 213, 207, 235, 210], OperandSize::Qword)
}

fn vporq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 297686608, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 173, 201, 235, 140, 74, 80, 86, 190, 17], OperandSize::Qword)
}

fn vporq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1354205773, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 210, 235, 28, 189, 77, 138, 183, 80], OperandSize::Qword)
}

