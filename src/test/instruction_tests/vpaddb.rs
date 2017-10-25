use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpaddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 252, 255], OperandSize::Dword)
}

fn vpaddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 359052425, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 252, 164, 192, 137, 180, 102, 21], OperandSize::Dword)
}

fn vpaddb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 252, 196], OperandSize::Qword)
}

fn vpaddb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1813967383, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 252, 20, 77, 23, 242, 30, 108], OperandSize::Qword)
}

fn vpaddb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 252, 226], OperandSize::Dword)
}

fn vpaddb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 252, 44, 240], OperandSize::Dword)
}

fn vpaddb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 252, 231], OperandSize::Qword)
}

fn vpaddb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1257989782, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 252, 44, 117, 150, 102, 251, 74], OperandSize::Qword)
}

fn vpaddb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 252, 220], OperandSize::Dword)
}

fn vpaddb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 214529910, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 137, 252, 164, 209, 118, 119, 201, 12], OperandSize::Dword)
}

fn vpaddb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 45, 142, 252, 235], OperandSize::Qword)
}

fn vpaddb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RCX, 2013571230, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 125, 134, 252, 185, 158, 168, 4, 120], OperandSize::Qword)
}

fn vpaddb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 252, 206], OperandSize::Dword)
}

fn vpaddb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 228995096, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 252, 152, 24, 48, 166, 13], OperandSize::Dword)
}

fn vpaddb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 45, 167, 252, 254], OperandSize::Qword)
}

fn vpaddb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1686539787, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 53, 174, 252, 164, 248, 11, 142, 134, 100], OperandSize::Qword)
}

