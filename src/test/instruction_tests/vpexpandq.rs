use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpexpandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 137, 226], OperandSize::Dword)
}

fn vpexpandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 137, 60, 185], OperandSize::Dword)
}

fn vpexpandq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 253, 142, 137, 216], OperandSize::Qword)
}

fn vpexpandq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(XMM21)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 253, 137, 137, 43], OperandSize::Qword)
}

fn vpexpandq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 137, 255], OperandSize::Dword)
}

fn vpexpandq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EDI, 839897993, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 137, 191, 137, 211, 15, 50], OperandSize::Dword)
}

fn vpexpandq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 253, 175, 137, 251], OperandSize::Qword)
}

fn vpexpandq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(YMM25)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 253, 173, 137, 15], OperandSize::Qword)
}

fn vpexpandq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 137, 247], OperandSize::Dword)
}

fn vpexpandq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 1564609162, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 137, 180, 71, 138, 10, 66, 93], OperandSize::Dword)
}

fn vpexpandq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 137, 233], OperandSize::Qword)
}

fn vpexpandq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 137, 54], OperandSize::Qword)
}

