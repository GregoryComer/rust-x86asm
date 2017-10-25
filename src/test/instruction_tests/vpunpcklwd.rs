use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpunpcklwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 97, 233], OperandSize::Dword)
}

fn vpunpcklwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 97, 49], OperandSize::Dword)
}

fn vpunpcklwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 97, 223], OperandSize::Qword)
}

fn vpunpcklwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RCX, 1257294520, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 97, 137, 184, 202, 240, 74], OperandSize::Qword)
}

fn vpunpcklwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 97, 248], OperandSize::Dword)
}

fn vpunpcklwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1585650620, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 97, 172, 211, 188, 27, 131, 94], OperandSize::Dword)
}

fn vpunpcklwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 97, 193], OperandSize::Qword)
}

fn vpunpcklwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDX, 1687285233, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 97, 170, 241, 237, 145, 100], OperandSize::Qword)
}

fn vpunpcklwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 97, 217], OperandSize::Dword)
}

fn vpunpcklwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 451551642, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 141, 97, 190, 154, 33, 234, 26], OperandSize::Dword)
}

fn vpunpcklwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 45, 130, 97, 233], OperandSize::Qword)
}

fn vpunpcklwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 201360153, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 13, 132, 97, 156, 95, 25, 131, 0, 12], OperandSize::Qword)
}

