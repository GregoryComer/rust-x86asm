use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpxorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 143, 239, 203], OperandSize::Dword)
}

fn vpxorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1664980165, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 239, 4, 181, 197, 148, 61, 99], OperandSize::Dword)
}

fn vpxorq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1835077706, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 154, 239, 12, 213, 74, 16, 97, 109], OperandSize::Dword)
}

fn vpxorq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 133, 135, 239, 208], OperandSize::Qword)
}

fn vpxorq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 157, 131, 239, 52, 126], OperandSize::Qword)
}

fn vpxorq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 253, 154, 239, 12, 250], OperandSize::Qword)
}

fn vpxorq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 175, 239, 221], OperandSize::Dword)
}

fn vpxorq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1413499724, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 175, 239, 60, 141, 76, 75, 64, 84], OperandSize::Dword)
}

fn vpxorq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 188, 239, 57], OperandSize::Dword)
}

fn vpxorq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 162, 239, 203], OperandSize::Qword)
}

fn vpxorq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 860698698, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 167, 239, 172, 126, 74, 56, 77, 51], OperandSize::Qword)
}

fn vpxorq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RBX, 72199666, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 213, 187, 239, 131, 242, 173, 77, 4], OperandSize::Qword)
}

fn vpxorq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 203, 239, 195], OperandSize::Dword)
}

fn vpxorq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 206, 239, 58], OperandSize::Dword)
}

fn vpxorq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 555901196, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 223, 239, 12, 213, 12, 97, 34, 33], OperandSize::Dword)
}

fn vpxorq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 213, 203, 239, 231], OperandSize::Qword)
}

fn vpxorq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 165, 194, 239, 56], OperandSize::Qword)
}

fn vpxorq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 141, 217, 239, 63], OperandSize::Qword)
}

