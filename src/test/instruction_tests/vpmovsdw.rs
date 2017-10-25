use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 35, 198], OperandSize::Dword)
}

fn vpmovsdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 35, 63], OperandSize::Dword)
}

fn vpmovsdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 126, 137, 35, 221], OperandSize::Qword)
}

fn vpmovsdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 35, 28, 86], OperandSize::Qword)
}

fn vpmovsdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 35, 194], OperandSize::Dword)
}

fn vpmovsdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 35, 36, 131], OperandSize::Dword)
}

fn vpmovsdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 126, 173, 35, 199], OperandSize::Qword)
}

fn vpmovsdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 994845078, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 35, 52, 197, 150, 33, 76, 59], OperandSize::Qword)
}

fn vpmovsdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 35, 226], OperandSize::Dword)
}

fn vpmovsdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 35, 11], OperandSize::Dword)
}

fn vpmovsdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 126, 204, 35, 192], OperandSize::Qword)
}

fn vpmovsdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 35, 18], OperandSize::Qword)
}

