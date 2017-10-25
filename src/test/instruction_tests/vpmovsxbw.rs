use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 245], OperandSize::Dword)
}

fn vpmovsxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ESI, 536468620, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 142, 140, 220, 249, 31], OperandSize::Dword)
}

fn vpmovsxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 251], OperandSize::Qword)
}

fn vpmovsxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 52, 91], OperandSize::Qword)
}

fn vpmovsxbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 208], OperandSize::Dword)
}

fn vpmovsxbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 44, 211], OperandSize::Dword)
}

fn vpmovsxbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 210], OperandSize::Qword)
}

fn vpmovsxbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 687058742, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 140, 64, 54, 175, 243, 40], OperandSize::Qword)
}

fn vpmovsxbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 32, 195], OperandSize::Dword)
}

fn vpmovsxbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 32, 44, 66], OperandSize::Dword)
}

fn vpmovsxbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 125, 141, 32, 251], OperandSize::Qword)
}

fn vpmovsxbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 140, 32, 52, 74], OperandSize::Qword)
}

fn vpmovsxbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 32, 233], OperandSize::Dword)
}

fn vpmovsxbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 449681267, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 32, 20, 93, 115, 151, 205, 26], OperandSize::Dword)
}

fn vpmovsxbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 170, 32, 198], OperandSize::Qword)
}

fn vpmovsxbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM19)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 169, 32, 28, 154], OperandSize::Qword)
}

fn vpmovsxbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 32, 235], OperandSize::Dword)
}

fn vpmovsxbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 1278973450, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 32, 148, 209, 10, 150, 59, 76], OperandSize::Dword)
}

fn vpmovsxbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 125, 203, 32, 205], OperandSize::Qword)
}

fn vpmovsxbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1609216529, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 32, 28, 245, 17, 178, 234, 95], OperandSize::Qword)
}

