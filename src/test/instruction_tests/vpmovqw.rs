use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 52, 253], OperandSize::Dword)
}

fn vpmovqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 52, 44, 118], OperandSize::Dword)
}

fn vpmovqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 126, 140, 52, 221], OperandSize::Qword)
}

fn vpmovqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 52, 34], OperandSize::Qword)
}

fn vpmovqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 52, 238], OperandSize::Dword)
}

fn vpmovqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 52, 46], OperandSize::Dword)
}

fn vpmovqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 126, 171, 52, 217], OperandSize::Qword)
}

fn vpmovqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 52, 7], OperandSize::Qword)
}

fn vpmovqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 52, 245], OperandSize::Dword)
}

fn vpmovqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 52, 15], OperandSize::Dword)
}

fn vpmovqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 126, 201, 52, 225], OperandSize::Qword)
}

fn vpmovqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1128586651, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 52, 164, 131, 155, 221, 68, 67], OperandSize::Qword)
}

