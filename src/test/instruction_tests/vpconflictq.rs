use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpconflictq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 196, 235], OperandSize::Dword)
}

fn vpconflictq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 1583916068, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 196, 176, 36, 164, 104, 94], OperandSize::Dword)
}

fn vpconflictq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 159, 196, 25], OperandSize::Dword)
}

fn vpconflictq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 253, 139, 196, 248], OperandSize::Qword)
}

fn vpconflictq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1558485165, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 196, 28, 157, 173, 152, 228, 92], OperandSize::Qword)
}

fn vpconflictq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM15)), operand2: Some(IndirectDisplaced(RSI, 990744704, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 253, 158, 196, 190, 128, 144, 13, 59], OperandSize::Qword)
}

fn vpconflictq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 196, 225], OperandSize::Dword)
}

fn vpconflictq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1617240137, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 196, 52, 93, 73, 32, 101, 96], OperandSize::Dword)
}

fn vpconflictq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EBX, 564315982, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 196, 179, 78, 199, 162, 33], OperandSize::Dword)
}

fn vpconflictq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 253, 169, 196, 232], OperandSize::Qword)
}

fn vpconflictq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1751079646, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 253, 169, 196, 148, 209, 222, 90, 95, 104], OperandSize::Qword)
}

fn vpconflictq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 405419947, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 253, 188, 196, 28, 157, 171, 55, 42, 24], OperandSize::Qword)
}

fn vpconflictq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 196, 202], OperandSize::Dword)
}

fn vpconflictq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 103492609, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 196, 20, 77, 1, 44, 43, 6], OperandSize::Dword)
}

fn vpconflictq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 223, 196, 38], OperandSize::Dword)
}

fn vpconflictq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 196, 244], OperandSize::Qword)
}

fn vpconflictq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1488814320, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 253, 207, 196, 156, 250, 240, 128, 189, 88], OperandSize::Qword)
}

fn vpconflictq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 219, 196, 28, 114], OperandSize::Qword)
}

