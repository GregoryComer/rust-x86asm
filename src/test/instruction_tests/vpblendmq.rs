use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpblendmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 100, 194], OperandSize::Dword)
}

fn vpblendmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 2001867030, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 138, 100, 156, 255, 22, 17, 82, 119], OperandSize::Dword)
}

fn vpblendmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 667300754, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 153, 100, 178, 146, 51, 198, 39], OperandSize::Dword)
}

fn vpblendmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 221, 140, 100, 198], OperandSize::Qword)
}

fn vpblendmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 433231114, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 173, 142, 100, 52, 133, 10, 149, 210, 25], OperandSize::Qword)
}

fn vpblendmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 159, 100, 52, 134], OperandSize::Qword)
}

fn vpblendmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 100, 252], OperandSize::Dword)
}

fn vpblendmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 175, 100, 28, 151], OperandSize::Dword)
}

fn vpblendmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 191, 100, 12, 223], OperandSize::Dword)
}

fn vpblendmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 165, 164, 100, 252], OperandSize::Qword)
}

fn vpblendmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RDX, 957072652, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 141, 161, 100, 178, 12, 197, 11, 57], OperandSize::Qword)
}

fn vpblendmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1075475030, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 189, 180, 100, 188, 194, 86, 114, 26, 64], OperandSize::Qword)
}

fn vpblendmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 202, 100, 210], OperandSize::Dword)
}

fn vpblendmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 202, 100, 34], OperandSize::Dword)
}

fn vpblendmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 217, 100, 4, 143], OperandSize::Dword)
}

fn vpblendmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 221, 202, 100, 215], OperandSize::Qword)
}

fn vpblendmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2129265241, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 189, 193, 100, 4, 93, 89, 2, 234, 126], OperandSize::Qword)
}

fn vpblendmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM11)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 165, 218, 100, 14], OperandSize::Qword)
}

