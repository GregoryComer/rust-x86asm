use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 143, 16, 213], OperandSize::Dword)
}

fn vpmovuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1160626391, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 16, 172, 191, 215, 192, 45, 69], OperandSize::Dword)
}

fn vpmovuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 126, 140, 16, 217], OperandSize::Qword)
}

fn vpmovuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 16, 54], OperandSize::Qword)
}

fn vpmovuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 16, 211], OperandSize::Dword)
}

fn vpmovuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectDisplaced(ESI, 1026208391, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 16, 134, 135, 178, 42, 61], OperandSize::Dword)
}

fn vpmovuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 126, 169, 16, 238], OperandSize::Qword)
}

fn vpmovuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 16, 47], OperandSize::Qword)
}

fn vpmovuswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 16, 245], OperandSize::Dword)
}

fn vpmovuswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 831517584, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 16, 156, 203, 144, 243, 143, 49], OperandSize::Dword)
}

fn vpmovuswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(YMM13)), operand2: Some(Direct(ZMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 126, 205, 16, 253], OperandSize::Qword)
}

fn vpmovuswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 16, 11], OperandSize::Qword)
}

