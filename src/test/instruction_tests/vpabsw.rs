use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpabsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 194], OperandSize::Dword)
}

fn vpabsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 52, 146], OperandSize::Dword)
}

fn vpabsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 224], OperandSize::Qword)
}

fn vpabsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 29, 4, 64], OperandSize::Qword)
}

fn vpabsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 237], OperandSize::Dword)
}

fn vpabsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 36, 193], OperandSize::Dword)
}

fn vpabsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 225], OperandSize::Qword)
}

fn vpabsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(RAX, 942947037, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 29, 128, 221, 58, 52, 56], OperandSize::Qword)
}

fn vpabsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 29, 199], OperandSize::Dword)
}

fn vpabsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 299809001, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 29, 156, 241, 233, 184, 222, 17], OperandSize::Dword)
}

fn vpabsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 142, 29, 195], OperandSize::Qword)
}

fn vpabsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 1611774107, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 29, 156, 249, 155, 184, 17, 96], OperandSize::Qword)
}

