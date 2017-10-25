use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrndscaleps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 143, 8, 244, 57], OperandSize::Dword)
}

fn vrndscaleps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 195423049, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 143, 8, 20, 69, 73, 235, 165, 11, 34], OperandSize::Dword)
}

fn vrndscaleps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 582103666, Some(OperandSize::Dword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 157, 8, 156, 241, 114, 50, 178, 34, 61], OperandSize::Dword)
}

fn vrndscaleps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM12)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 83, 125, 140, 8, 244, 18], OperandSize::Qword)
}

fn vrndscaleps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 2010579195, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 141, 8, 164, 215, 251, 0, 215, 119, 111], OperandSize::Qword)
}

fn vrndscaleps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 874115094, Some(OperandSize::Dword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 125, 155, 8, 28, 197, 22, 240, 25, 52, 35], OperandSize::Qword)
}

fn vrndscaleps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 8, 198, 62], OperandSize::Dword)
}

fn vrndscaleps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EDI, 124907023, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 172, 8, 151, 15, 238, 113, 7, 0], OperandSize::Dword)
}

fn vrndscaleps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 187, 8, 20, 82, 82], OperandSize::Dword)
}

fn vrndscaleps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 125, 172, 8, 219, 12], OperandSize::Qword)
}

fn vrndscaleps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM27)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 125, 175, 8, 30, 51], OperandSize::Qword)
}

fn vrndscaleps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(YMM30)), operand2: Some(IndirectDisplaced(RSI, 917424064, Some(OperandSize::Dword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 125, 191, 8, 182, 192, 199, 174, 54, 88], OperandSize::Qword)
}

fn vrndscaleps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 155, 8, 228, 5], OperandSize::Dword)
}

fn vrndscaleps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 894010830, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 206, 8, 28, 93, 206, 133, 73, 53, 47], OperandSize::Dword)
}

fn vrndscaleps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 972909601, Some(OperandSize::Dword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 221, 8, 172, 222, 33, 108, 253, 57, 126], OperandSize::Dword)
}

fn vrndscaleps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 159, 8, 224, 104], OperandSize::Qword)
}

fn vrndscaleps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM28)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 125, 203, 8, 39, 29], OperandSize::Qword)
}

fn vrndscaleps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPS, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 125, 222, 8, 60, 127, 95], OperandSize::Qword)
}

