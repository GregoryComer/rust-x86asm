use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovzxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 203], OperandSize::Dword)
}

fn vpmovzxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ECX, 948217790, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 153, 190, 167, 132, 56], OperandSize::Dword)
}

fn vpmovzxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 214], OperandSize::Qword)
}

fn vpmovzxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1077894059, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 156, 240, 171, 91, 63, 64], OperandSize::Qword)
}

fn vpmovzxwq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 201], OperandSize::Dword)
}

fn vpmovzxwq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 38716167, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 140, 193, 7, 195, 78, 2], OperandSize::Dword)
}

fn vpmovzxwq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 196], OperandSize::Qword)
}

fn vpmovzxwq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 571249124, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 20, 221, 228, 145, 12, 34], OperandSize::Qword)
}

fn vpmovzxwq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 52, 240], OperandSize::Dword)
}

fn vpmovzxwq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 52, 52, 145], OperandSize::Dword)
}

fn vpmovzxwq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 138, 52, 215], OperandSize::Qword)
}

fn vpmovzxwq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM13)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 140, 52, 46], OperandSize::Qword)
}

fn vpmovzxwq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 52, 210], OperandSize::Dword)
}

fn vpmovzxwq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 52, 40], OperandSize::Dword)
}

fn vpmovzxwq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 125, 173, 52, 200], OperandSize::Qword)
}

fn vpmovzxwq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 52, 25], OperandSize::Qword)
}

fn vpmovzxwq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 52, 220], OperandSize::Dword)
}

fn vpmovzxwq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 709691844, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 52, 172, 64, 196, 9, 77, 42], OperandSize::Dword)
}

fn vpmovzxwq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 125, 203, 52, 196], OperandSize::Qword)
}

fn vpmovzxwq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectDisplaced(RDX, 1476404265, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 206, 52, 186, 41, 36, 0, 88], OperandSize::Qword)
}

