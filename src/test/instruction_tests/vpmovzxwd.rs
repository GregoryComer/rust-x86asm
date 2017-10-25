use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovzxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 201], OperandSize::Dword)
}

fn vpmovzxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 54], OperandSize::Dword)
}

fn vpmovzxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 204], OperandSize::Qword)
}

fn vpmovzxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 23], OperandSize::Qword)
}

fn vpmovzxwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 245], OperandSize::Dword)
}

fn vpmovzxwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 2014313199, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 12, 245, 239, 250, 15, 120], OperandSize::Dword)
}

fn vpmovzxwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 204], OperandSize::Qword)
}

fn vpmovzxwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 32], OperandSize::Qword)
}

fn vpmovzxwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 51, 207], OperandSize::Dword)
}

fn vpmovzxwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1554327617, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 51, 28, 77, 65, 40, 165, 92], OperandSize::Dword)
}

fn vpmovzxwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 125, 138, 51, 220], OperandSize::Qword)
}

fn vpmovzxwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1958351550, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 137, 51, 188, 201, 190, 18, 186, 116], OperandSize::Qword)
}

fn vpmovzxwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 51, 224], OperandSize::Dword)
}

fn vpmovzxwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 530499369, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 51, 36, 245, 41, 199, 158, 31], OperandSize::Dword)
}

fn vpmovzxwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 125, 175, 51, 202], OperandSize::Qword)
}

fn vpmovzxwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 51, 51], OperandSize::Qword)
}

fn vpmovzxwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 51, 232], OperandSize::Dword)
}

fn vpmovzxwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 51, 44, 199], OperandSize::Dword)
}

fn vpmovzxwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 125, 207, 51, 254], OperandSize::Qword)
}

fn vpmovzxwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(RBX, 202452725, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 51, 147, 245, 46, 17, 12], OperandSize::Qword)
}

