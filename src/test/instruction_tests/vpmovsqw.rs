use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 36, 193], OperandSize::Dword)
}

fn vpmovsqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectDisplaced(EDX, 1699146322, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 36, 162, 82, 234, 70, 101], OperandSize::Dword)
}

fn vpmovsqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 36, 216], OperandSize::Qword)
}

fn vpmovsqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 575753050, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 36, 28, 197, 90, 75, 81, 34], OperandSize::Qword)
}

fn vpmovsqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 172, 36, 198], OperandSize::Dword)
}

fn vpmovsqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledDisplaced(EBX, Four, 922818178, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 36, 52, 157, 130, 22, 1, 55], OperandSize::Dword)
}

fn vpmovsqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 126, 175, 36, 218], OperandSize::Qword)
}

fn vpmovsqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 976760740, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 36, 52, 245, 164, 47, 56, 58], OperandSize::Qword)
}

fn vpmovsqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 36, 235], OperandSize::Dword)
}

fn vpmovsqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1270323608, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 36, 188, 65, 152, 153, 183, 75], OperandSize::Dword)
}

fn vpmovsqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 126, 204, 36, 227], OperandSize::Qword)
}

fn vpmovsqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectDisplaced(RCX, 617841596, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 36, 145, 188, 131, 211, 36], OperandSize::Qword)
}

