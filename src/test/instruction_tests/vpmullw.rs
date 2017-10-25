use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmullw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 213, 249], OperandSize::Dword)
}

fn vpmullw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1822682341, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 213, 164, 146, 229, 236, 163, 108], OperandSize::Dword)
}

fn vpmullw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 213, 209], OperandSize::Qword)
}

fn vpmullw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 94986991, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 213, 172, 87, 239, 98, 169, 5], OperandSize::Qword)
}

fn vpmullw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 213, 192], OperandSize::Dword)
}

fn vpmullw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 213, 17], OperandSize::Dword)
}

fn vpmullw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 213, 207], OperandSize::Qword)
}

fn vpmullw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 213, 28, 90], OperandSize::Qword)
}

fn vpmullw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 213, 248], OperandSize::Dword)
}

fn vpmullw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EBX, 237191658, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 213, 139, 234, 65, 35, 14], OperandSize::Dword)
}

fn vpmullw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 101, 133, 213, 214], OperandSize::Qword)
}

fn vpmullw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM16)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 135, 213, 16], OperandSize::Qword)
}

fn vpmullw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 169, 213, 219], OperandSize::Dword)
}

fn vpmullw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 747756023, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 170, 213, 188, 86, 247, 217, 145, 44], OperandSize::Dword)
}

fn vpmullw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 69, 172, 213, 251], OperandSize::Qword)
}

fn vpmullw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectDisplaced(RDX, 729778980, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 13, 171, 213, 154, 36, 139, 127, 43], OperandSize::Qword)
}

fn vpmullw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 213, 197], OperandSize::Dword)
}

fn vpmullw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1175896385, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 213, 60, 149, 65, 193, 22, 70], OperandSize::Dword)
}

fn vpmullw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 109, 194, 213, 213], OperandSize::Qword)
}

fn vpmullw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLW, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 61, 196, 213, 52, 91], OperandSize::Qword)
}

