use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtdq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 212], OperandSize::Dword)
}

fn vcvtdq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1716861490, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 140, 134, 50, 58, 85, 102], OperandSize::Dword)
}

fn vcvtdq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 214], OperandSize::Qword)
}

fn vcvtdq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1844211556, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 164, 250, 100, 111, 236, 109], OperandSize::Qword)
}

fn vcvtdq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 215], OperandSize::Dword)
}

fn vcvtdq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 56], OperandSize::Dword)
}

fn vcvtdq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 255], OperandSize::Qword)
}

fn vcvtdq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1718466530, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 36, 149, 226, 183, 109, 102], OperandSize::Qword)
}

fn vcvtdq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 91, 204], OperandSize::Dword)
}

fn vcvtdq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 91, 4, 202], OperandSize::Dword)
}

fn vcvtdq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 124, 140, 91, 214], OperandSize::Qword)
}

fn vcvtdq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM12)), operand2: Some(IndirectDisplaced(RCX, 1430654405, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 124, 137, 91, 161, 197, 13, 70, 85], OperandSize::Qword)
}

fn vcvtdq2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 91, 226], OperandSize::Dword)
}

fn vcvtdq2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 325894591, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 169, 91, 28, 141, 191, 193, 108, 19], OperandSize::Dword)
}

fn vcvtdq2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 124, 172, 91, 201], OperandSize::Qword)
}

fn vcvtdq2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM29)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 124, 175, 91, 44, 219], OperandSize::Qword)
}

fn vcvtdq2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 157, 91, 209], OperandSize::Dword)
}

fn vcvtdq2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EBX, 242122022, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 91, 171, 38, 125, 110, 14], OperandSize::Dword)
}

fn vcvtdq2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 124, 249, 91, 252], OperandSize::Qword)
}

fn vcvtdq2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 124, 201, 91, 44, 115], OperandSize::Qword)
}

