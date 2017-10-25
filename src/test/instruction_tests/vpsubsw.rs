use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 233, 194], OperandSize::Dword)
}

fn vpsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 560988330, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 233, 129, 170, 0, 112, 33], OperandSize::Dword)
}

fn vpsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 233, 218], OperandSize::Qword)
}

fn vpsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 233, 12, 217], OperandSize::Qword)
}

fn vpsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 233, 215], OperandSize::Dword)
}

fn vpsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 233, 46], OperandSize::Dword)
}

fn vpsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 233, 194], OperandSize::Qword)
}

fn vpsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 33199425, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 233, 140, 83, 65, 149, 250, 1], OperandSize::Qword)
}

fn vpsubsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 142, 233, 202], OperandSize::Dword)
}

fn vpsubsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 233, 44, 78], OperandSize::Dword)
}

fn vpsubsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 69, 131, 233, 221], OperandSize::Qword)
}

fn vpsubsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1537093511, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 37, 142, 233, 44, 157, 135, 47, 158, 91], OperandSize::Qword)
}

fn vpsubsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 172, 233, 211], OperandSize::Dword)
}

fn vpsubsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 233, 55], OperandSize::Dword)
}

fn vpsubsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 13, 173, 233, 219], OperandSize::Qword)
}

fn vpsubsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM24)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 61, 164, 233, 24], OperandSize::Qword)
}

