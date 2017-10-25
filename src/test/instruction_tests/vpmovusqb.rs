use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovusqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 18, 238], OperandSize::Dword)
}

fn vpmovusqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectDisplaced(EDX, 619749163, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 18, 130, 43, 159, 240, 36], OperandSize::Dword)
}

fn vpmovusqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 126, 143, 18, 235], OperandSize::Qword)
}

fn vpmovusqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1433783856, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 18, 4, 69, 48, 206, 117, 85], OperandSize::Qword)
}

fn vpmovusqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 18, 229], OperandSize::Dword)
}

fn vpmovusqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 115915574, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 18, 140, 75, 54, 187, 232, 6], OperandSize::Dword)
}

fn vpmovusqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM30)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 126, 174, 18, 214], OperandSize::Qword)
}

fn vpmovusqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledDisplaced(RDI, Four, 874776764, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 18, 36, 189, 188, 8, 36, 52], OperandSize::Qword)
}

fn vpmovusqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 204, 18, 241], OperandSize::Dword)
}

fn vpmovusqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1773280297, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 18, 36, 245, 41, 28, 178, 105], OperandSize::Dword)
}

fn vpmovusqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 126, 202, 18, 237], OperandSize::Qword)
}

fn vpmovusqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 18, 20, 246], OperandSize::Qword)
}

