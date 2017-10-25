use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermt2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 126, 250], OperandSize::Dword)
}

fn vpermt2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 126, 40], OperandSize::Dword)
}

fn vpermt2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 1673834900, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 158, 126, 191, 148, 177, 196, 99], OperandSize::Dword)
}

fn vpermt2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 237, 129, 126, 206], OperandSize::Qword)
}

fn vpermt2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 134, 126, 7], OperandSize::Qword)
}

fn vpermt2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 149, 157, 126, 35], OperandSize::Qword)
}

fn vpermt2q_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 172, 126, 192], OperandSize::Dword)
}

fn vpermt2q_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 420477122, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 171, 126, 182, 194, 248, 15, 25], OperandSize::Dword)
}

fn vpermt2q_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 190, 126, 10], OperandSize::Dword)
}

fn vpermt2q_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 213, 166, 126, 196], OperandSize::Qword)
}

fn vpermt2q_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 205, 174, 126, 28, 155], OperandSize::Qword)
}

fn vpermt2q_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1650928561, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 165, 179, 126, 20, 133, 177, 43, 103, 98], OperandSize::Qword)
}

fn vpermt2q_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 201, 126, 227], OperandSize::Dword)
}

fn vpermt2q_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDI, 1667897615, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 203, 126, 135, 15, 25, 106, 99], OperandSize::Dword)
}

fn vpermt2q_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 915307164, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 126, 148, 179, 156, 122, 142, 54], OperandSize::Dword)
}

fn vpermt2q_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 181, 206, 126, 204], OperandSize::Qword)
}

fn vpermt2q_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RDX, 303828984, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 197, 203, 126, 178, 248, 15, 28, 18], OperandSize::Qword)
}

fn vpermt2q_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1224876451, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 205, 218, 126, 140, 186, 163, 33, 2, 73], OperandSize::Qword)
}

