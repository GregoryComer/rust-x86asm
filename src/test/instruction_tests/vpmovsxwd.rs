use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 213], OperandSize::Dword)
}

fn vpmovsxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 124041848, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 28, 69, 120, 186, 100, 7], OperandSize::Dword)
}

fn vpmovsxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 218], OperandSize::Qword)
}

fn vpmovsxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 12, 193], OperandSize::Qword)
}

fn vpmovsxwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 228], OperandSize::Dword)
}

fn vpmovsxwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 48], OperandSize::Dword)
}

fn vpmovsxwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 246], OperandSize::Qword)
}

fn vpmovsxwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 47], OperandSize::Qword)
}

fn vpmovsxwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 35, 212], OperandSize::Dword)
}

fn vpmovsxwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 2015746710, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 35, 145, 150, 218, 37, 120], OperandSize::Dword)
}

fn vpmovsxwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 125, 143, 35, 232], OperandSize::Qword)
}

fn vpmovsxwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM16)), operand2: Some(IndirectDisplaced(RDX, 1888961629, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 138, 35, 130, 93, 68, 151, 112], OperandSize::Qword)
}

fn vpmovsxwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 35, 224], OperandSize::Dword)
}

fn vpmovsxwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 35, 42], OperandSize::Dword)
}

fn vpmovsxwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 169, 35, 246], OperandSize::Qword)
}

fn vpmovsxwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 2069523878, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 35, 164, 139, 166, 109, 90, 123], OperandSize::Qword)
}

fn vpmovsxwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 35, 204], OperandSize::Dword)
}

fn vpmovsxwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 819760606, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 35, 28, 85, 222, 141, 220, 48], OperandSize::Dword)
}

fn vpmovsxwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 125, 203, 35, 230], OperandSize::Qword)
}

fn vpmovsxwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM10)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 203, 35, 23], OperandSize::Qword)
}

