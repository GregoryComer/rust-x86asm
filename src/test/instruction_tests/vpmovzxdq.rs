use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovzxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 220], OperandSize::Dword)
}

fn vpmovzxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1848943893, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 44, 133, 21, 165, 52, 110], OperandSize::Dword)
}

fn vpmovzxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 207], OperandSize::Qword)
}

fn vpmovzxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 53, 36, 243], OperandSize::Qword)
}

fn vpmovzxdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 198], OperandSize::Dword)
}

fn vpmovzxdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 57], OperandSize::Dword)
}

fn vpmovzxdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 213], OperandSize::Qword)
}

fn vpmovzxdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXDQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 53, 1], OperandSize::Qword)
}

