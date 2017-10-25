use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn valignq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 237, 141, 3, 240, 75], OperandSize::Dword)
}

fn valignq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 1131579328, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 245, 139, 3, 128, 192, 135, 114, 67, 28], OperandSize::Dword)
}

fn valignq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 789230963, Some(OperandSize::Qword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 205, 159, 3, 156, 182, 115, 181, 10, 47, 21], OperandSize::Dword)
}

fn valignq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM27)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 131, 229, 141, 3, 219, 6], OperandSize::Qword)
}

fn valignq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1889471967, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 165, 133, 3, 52, 85, 223, 13, 159, 112, 11], OperandSize::Qword)
}

fn valignq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 157, 157, 3, 44, 190, 15], OperandSize::Qword)
}

fn valignq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 170, 3, 249, 95], OperandSize::Dword)
}

fn valignq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 205, 170, 3, 26, 114], OperandSize::Dword)
}

fn valignq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 1371757085, Some(OperandSize::Qword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 245, 188, 3, 176, 29, 90, 195, 81, 41], OperandSize::Dword)
}

fn valignq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM9)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 83, 133, 172, 3, 209, 96], OperandSize::Qword)
}

fn valignq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM24)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 189, 162, 3, 22, 11], OperandSize::Qword)
}

fn valignq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 842954339, Some(OperandSize::Qword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 205, 177, 3, 172, 127, 99, 118, 62, 50, 36], OperandSize::Qword)
}

fn valignq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 206, 3, 198, 63], OperandSize::Dword)
}

fn valignq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 205, 202, 3, 12, 255, 33], OperandSize::Dword)
}

fn valignq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 245, 223, 3, 48, 67], OperandSize::Dword)
}

fn valignq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM22)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 205, 197, 3, 222, 62], OperandSize::Qword)
}

fn valignq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RDI, 1641057420, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 141, 194, 3, 167, 140, 140, 208, 97, 68], OperandSize::Qword)
}

fn valignq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 229, 212, 3, 4, 119, 8], OperandSize::Qword)
}

