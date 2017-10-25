use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vplzcntq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 68, 202], OperandSize::Dword)
}

fn vplzcntq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 68, 23], OperandSize::Dword)
}

fn vplzcntq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDI, 6046811, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 153, 68, 159, 91, 68, 92, 0], OperandSize::Dword)
}

fn vplzcntq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 253, 142, 68, 203], OperandSize::Qword)
}

fn vplzcntq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1395015366, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 253, 138, 68, 172, 67, 198, 62, 38, 83], OperandSize::Qword)
}

fn vplzcntq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1590321450, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 253, 157, 68, 148, 71, 42, 97, 202, 94], OperandSize::Qword)
}

fn vplzcntq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 68, 202], OperandSize::Dword)
}

fn vplzcntq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 512493704, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 68, 188, 89, 136, 8, 140, 30], OperandSize::Dword)
}

fn vplzcntq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 190, 68, 52, 73], OperandSize::Dword)
}

fn vplzcntq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 253, 171, 68, 204], OperandSize::Qword)
}

fn vplzcntq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM8)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 253, 172, 68, 7], OperandSize::Qword)
}

fn vplzcntq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 998395644, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 253, 190, 68, 172, 150, 252, 78, 130, 59], OperandSize::Qword)
}

fn vplzcntq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 68, 246], OperandSize::Dword)
}

fn vplzcntq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EBX, 827145319, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 68, 131, 103, 60, 77, 49], OperandSize::Dword)
}

fn vplzcntq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(EDX, 1446850019, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 223, 68, 138, 227, 45, 61, 86], OperandSize::Dword)
}

fn vplzcntq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 253, 204, 68, 216], OperandSize::Qword)
}

fn vplzcntq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 206, 68, 44, 194], OperandSize::Qword)
}

fn vplzcntq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1753424205, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 253, 219, 68, 52, 117, 77, 33, 131, 104], OperandSize::Qword)
}

