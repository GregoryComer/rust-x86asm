use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpminsq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 142, 57, 229], OperandSize::Dword)
}

fn vpminsq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 57, 20, 243], OperandSize::Dword)
}

fn vpminsq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 158, 57, 10], OperandSize::Dword)
}

fn vpminsq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 205, 138, 57, 243], OperandSize::Qword)
}

fn vpminsq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1560768921, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 213, 129, 57, 4, 205, 153, 113, 7, 93], OperandSize::Qword)
}

fn vpminsq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RDX, 823479129, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 157, 156, 57, 162, 89, 75, 21, 49], OperandSize::Qword)
}

fn vpminsq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 170, 57, 210], OperandSize::Dword)
}

fn vpminsq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 1188200135, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 170, 57, 188, 128, 199, 126, 210, 70], OperandSize::Dword)
}

fn vpminsq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 2037999638, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 190, 57, 128, 22, 104, 121, 121], OperandSize::Dword)
}

fn vpminsq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 253, 172, 57, 240], OperandSize::Qword)
}

fn vpminsq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 189, 169, 57, 36, 66], OperandSize::Qword)
}

fn vpminsq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1541586314, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 133, 178, 57, 180, 123, 138, 189, 226, 91], OperandSize::Qword)
}

fn vpminsq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 206, 57, 244], OperandSize::Dword)
}

fn vpminsq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 737480275, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 205, 57, 52, 253, 83, 14, 245, 43], OperandSize::Dword)
}

fn vpminsq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ECX, 424812969, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 57, 153, 169, 33, 82, 25], OperandSize::Dword)
}

fn vpminsq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 149, 206, 57, 252], OperandSize::Qword)
}

fn vpminsq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 173, 199, 57, 44, 179], OperandSize::Qword)
}

fn vpminsq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 189, 219, 57, 36, 127], OperandSize::Qword)
}

