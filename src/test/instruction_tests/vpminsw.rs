use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpminsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 234, 238], OperandSize::Dword)
}

fn vpminsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 957798777, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 234, 20, 77, 121, 217, 22, 57], OperandSize::Dword)
}

fn vpminsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 234, 195], OperandSize::Qword)
}

fn vpminsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RCX, 1777271856, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 234, 185, 48, 4, 239, 105], OperandSize::Qword)
}

fn vpminsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 234, 224], OperandSize::Dword)
}

fn vpminsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 234, 17], OperandSize::Dword)
}

fn vpminsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 234, 244], OperandSize::Qword)
}

fn vpminsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 234, 28, 254], OperandSize::Qword)
}

fn vpminsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 138, 234, 252], OperandSize::Dword)
}

fn vpminsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 234, 46], OperandSize::Dword)
}

fn vpminsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 69, 140, 234, 215], OperandSize::Qword)
}

fn vpminsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 61, 133, 234, 12, 184], OperandSize::Qword)
}

fn vpminsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 175, 234, 255], OperandSize::Dword)
}

fn vpminsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1181656573, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 175, 234, 44, 213, 253, 165, 110, 70], OperandSize::Dword)
}

fn vpminsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 13, 173, 234, 199], OperandSize::Qword)
}

fn vpminsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM11)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 37, 171, 234, 33], OperandSize::Qword)
}

fn vpminsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 234, 222], OperandSize::Dword)
}

fn vpminsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EBX, 842465771, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 201, 234, 139, 235, 1, 55, 50], OperandSize::Dword)
}

fn vpminsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 45, 207, 234, 237], OperandSize::Qword)
}

fn vpminsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1261752462, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 202, 234, 172, 240, 142, 208, 52, 75], OperandSize::Qword)
}

