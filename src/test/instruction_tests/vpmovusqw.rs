use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovusqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 20, 204], OperandSize::Dword)
}

fn vpmovusqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 20, 46], OperandSize::Dword)
}

fn vpmovusqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 126, 143, 20, 208], OperandSize::Qword)
}

fn vpmovusqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 20, 44, 94], OperandSize::Qword)
}

fn vpmovusqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 20, 213], OperandSize::Dword)
}

fn vpmovusqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 554013602, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 20, 164, 241, 162, 147, 5, 33], OperandSize::Dword)
}

fn vpmovusqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 126, 173, 20, 234], OperandSize::Qword)
}

fn vpmovusqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 20, 22], OperandSize::Qword)
}

fn vpmovusqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 20, 196], OperandSize::Dword)
}

fn vpmovusqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 309231232, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 20, 12, 205, 128, 126, 110, 18], OperandSize::Dword)
}

fn vpmovusqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 126, 206, 20, 193], OperandSize::Qword)
}

fn vpmovusqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 20, 52, 198], OperandSize::Qword)
}

