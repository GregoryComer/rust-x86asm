use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 234], OperandSize::Dword)
}

fn vpmovsxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1715754477, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 28, 253, 237, 85, 68, 102], OperandSize::Dword)
}

fn vpmovsxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 193], OperandSize::Qword)
}

fn vpmovsxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 36, 95], OperandSize::Qword)
}

fn vpmovsxwq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 234], OperandSize::Dword)
}

fn vpmovsxwq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 12, 200], OperandSize::Dword)
}

fn vpmovsxwq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 250], OperandSize::Qword)
}

fn vpmovsxwq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 20, 251], OperandSize::Qword)
}

fn vpmovsxwq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 36, 246], OperandSize::Dword)
}

fn vpmovsxwq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 36, 58], OperandSize::Dword)
}

fn vpmovsxwq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 125, 139, 36, 230], OperandSize::Qword)
}

fn vpmovsxwq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1605617341, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 36, 164, 192, 189, 198, 179, 95], OperandSize::Qword)
}

fn vpmovsxwq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 36, 216], OperandSize::Dword)
}

fn vpmovsxwq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 2028111353, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 36, 36, 189, 249, 133, 226, 120], OperandSize::Dword)
}

fn vpmovsxwq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 125, 171, 36, 215], OperandSize::Qword)
}

fn vpmovsxwq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM8)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 172, 36, 6], OperandSize::Qword)
}

fn vpmovsxwq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 36, 226], OperandSize::Dword)
}

fn vpmovsxwq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 36, 44, 218], OperandSize::Dword)
}

fn vpmovsxwq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 204, 36, 255], OperandSize::Qword)
}

fn vpmovsxwq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM28)), operand2: Some(IndirectDisplaced(RBX, 1834975569, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 204, 36, 163, 81, 129, 95, 109], OperandSize::Qword)
}

