use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 139, 33, 208], OperandSize::Dword)
}

fn vpmovsdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 33, 10], OperandSize::Dword)
}

fn vpmovsdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 126, 143, 33, 234], OperandSize::Qword)
}

fn vpmovsdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 33, 60, 65], OperandSize::Qword)
}

fn vpmovsdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 33, 223], OperandSize::Dword)
}

fn vpmovsdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectDisplaced(EAX, 1361820623, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 33, 152, 207, 187, 43, 81], OperandSize::Dword)
}

fn vpmovsdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM10)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 126, 171, 33, 218], OperandSize::Qword)
}

fn vpmovsdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 33, 32], OperandSize::Qword)
}

fn vpmovsdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 202, 33, 228], OperandSize::Dword)
}

fn vpmovsdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(IndirectDisplaced(EAX, 1677260254, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 33, 176, 222, 245, 248, 99], OperandSize::Dword)
}

fn vpmovsdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Direct(XMM8)), operand2: Some(Direct(ZMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 126, 205, 33, 248], OperandSize::Qword)
}

fn vpmovsdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDB, operand1: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 33, 6], OperandSize::Qword)
}

