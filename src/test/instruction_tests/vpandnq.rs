use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpandnq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 141, 223, 253], OperandSize::Dword)
}

fn vpandnq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 1237086395, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 139, 223, 153, 187, 112, 188, 73], OperandSize::Dword)
}

fn vpandnq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 154, 223, 24], OperandSize::Dword)
}

fn vpandnq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 157, 137, 223, 222], OperandSize::Qword)
}

fn vpandnq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectDisplaced(RAX, 1682737928, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 245, 133, 223, 184, 8, 139, 76, 100], OperandSize::Qword)
}

fn vpandnq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 253, 145, 223, 60, 202], OperandSize::Qword)
}

fn vpandnq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 170, 223, 209], OperandSize::Dword)
}

fn vpandnq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1205915074, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 245, 171, 223, 180, 254, 194, 205, 224, 71], OperandSize::Dword)
}

fn vpandnq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 191, 223, 12, 192], OperandSize::Dword)
}

fn vpandnq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 189, 171, 223, 227], OperandSize::Qword)
}

fn vpandnq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 197, 169, 223, 52, 217], OperandSize::Qword)
}

fn vpandnq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RDX, 937793720, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 245, 181, 223, 186, 184, 152, 229, 55], OperandSize::Qword)
}

fn vpandnq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 207, 223, 236], OperandSize::Dword)
}

fn vpandnq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1549607496, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 223, 44, 69, 72, 34, 93, 92], OperandSize::Dword)
}

fn vpandnq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1212383450, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 220, 223, 148, 88, 218, 128, 67, 72], OperandSize::Dword)
}

fn vpandnq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 229, 207, 223, 238], OperandSize::Qword)
}

fn vpandnq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 133, 203, 223, 12, 208], OperandSize::Qword)
}

fn vpandnq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 205, 214, 223, 54], OperandSize::Qword)
}

