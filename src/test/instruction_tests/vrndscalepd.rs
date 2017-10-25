use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrndscalepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 143, 9, 196, 42], OperandSize::Dword)
}

fn vrndscalepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1178595764, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 138, 9, 148, 191, 180, 241, 63, 70, 127], OperandSize::Dword)
}

fn vrndscalepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 154, 9, 36, 159, 8], OperandSize::Dword)
}

fn vrndscalepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM15)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 211, 253, 141, 9, 255, 113], OperandSize::Qword)
}

fn vrndscalepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM22)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 253, 138, 9, 55, 55], OperandSize::Qword)
}

fn vrndscalepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 227, 253, 157, 9, 60, 80, 120], OperandSize::Qword)
}

fn vrndscalepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 9, 217, 84], OperandSize::Dword)
}

fn vrndscalepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 973000224, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 9, 188, 191, 32, 206, 254, 57, 0], OperandSize::Dword)
}

fn vrndscalepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDI, 708297624, Some(OperandSize::Qword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 191, 9, 143, 152, 195, 55, 42, 44], OperandSize::Dword)
}

fn vrndscalepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 253, 175, 9, 248, 18], OperandSize::Qword)
}

fn vrndscalepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM20)), operand2: Some(IndirectDisplaced(RDX, 721220730, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 253, 171, 9, 162, 122, 244, 252, 42, 71], OperandSize::Qword)
}

fn vrndscalepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM12)), operand2: Some(IndirectDisplaced(RDI, 257209093, Some(OperandSize::Qword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 253, 187, 9, 167, 5, 179, 84, 15, 125], OperandSize::Qword)
}

fn vrndscalepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 159, 9, 254, 117], OperandSize::Dword)
}

fn vrndscalepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1994312144, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 9, 156, 142, 208, 201, 222, 118, 98], OperandSize::Dword)
}

fn vrndscalepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 219, 9, 28, 139, 71], OperandSize::Dword)
}

fn vrndscalepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM29)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 147, 253, 156, 9, 197, 55], OperandSize::Qword)
}

fn vrndscalepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(RCX, 1708200429, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 206, 9, 177, 237, 17, 209, 101, 43], OperandSize::Qword)
}

fn vrndscalepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 771152941, Some(OperandSize::Qword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 253, 220, 9, 12, 133, 45, 220, 246, 45, 26], OperandSize::Qword)
}

