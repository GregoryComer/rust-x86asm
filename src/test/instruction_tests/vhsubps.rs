use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vhsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 125, 210], OperandSize::Dword)
}

fn vhsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 125, 28, 89], OperandSize::Dword)
}

fn vhsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 125, 239], OperandSize::Qword)
}

fn vhsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 650506272, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 125, 152, 32, 240, 197, 38], OperandSize::Qword)
}

fn vhsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 223, 125, 235], OperandSize::Dword)
}

fn vhsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDI, 621773072, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 125, 151, 16, 129, 15, 37], OperandSize::Dword)
}

fn vhsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 207, 125, 207], OperandSize::Qword)
}

fn vhsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VHSUBPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RDX, 549119035, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 247, 125, 154, 59, 228, 186, 32], OperandSize::Qword)
}

