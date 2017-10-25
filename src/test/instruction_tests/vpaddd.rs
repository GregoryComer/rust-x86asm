use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 254, 215], OperandSize::Dword)
}

fn vpaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 254, 58], OperandSize::Dword)
}

fn vpaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 254, 212], OperandSize::Qword)
}

fn vpaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 254, 44, 208], OperandSize::Qword)
}

fn vpaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 254, 238], OperandSize::Dword)
}

fn vpaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 254, 42], OperandSize::Dword)
}

fn vpaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 254, 251], OperandSize::Qword)
}

fn vpaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 461488558, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 254, 188, 223, 174, 193, 129, 27], OperandSize::Qword)
}

fn vpaddd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 254, 226], OperandSize::Dword)
}

fn vpaddd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 257864921, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 254, 159, 217, 180, 94, 15], OperandSize::Dword)
}

fn vpaddd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDI, 1543471959, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 158, 254, 151, 87, 131, 255, 91], OperandSize::Dword)
}

fn vpaddd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 125, 142, 254, 235], OperandSize::Qword)
}

fn vpaddd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1838390617, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 93, 129, 254, 4, 69, 89, 157, 147, 109], OperandSize::Qword)
}

fn vpaddd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RSI, 765367380, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 109, 157, 254, 174, 84, 148, 158, 45], OperandSize::Qword)
}

